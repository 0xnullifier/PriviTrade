use std::time::{SystemTime, UNIX_EPOCH};

use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env, serde_json};
use calimero_storage::collections::{UnorderedMap, Vector};

#[app::state(emits = Event)]
#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct AppState {
    messages: UnorderedMap<ProposalId, Vector<Message>>,
    perpetual_order_book: UnorderedMap<TokenId, PerpetualOrderBook>,
}

pub type TokenId = [u8; 4];
pub type TraderId = [u8; 8];
pub type Price = [u8; 8];

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Message {
    id: String,
    proposal_id: String,
    author: String,
    text: String,
    created_at: String,
}

fn round_price(price: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (price * factor).round() / factor
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum Side {
    Long,
    Short,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Position {
    size: f64,
    entry_price: f64,
    leverage: f64,
    liquidation_price: f64,
    unrealized_pnl: f64,
    margin: f64,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Order {
    id: u64,
    trader_id: TraderId,
    price: f64,
    size: f64,
    side: Side,
    order_type: OrderType,
    leverage: f64,
    timestamp: u64,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Trade {
    id: u64,
    maker_order_id: u64,
    taker_order_id: u64,
    price: f64,
    size: f64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct PerpetualOrderBook {
    next_id: u64,
    bids: UnorderedMap<Price, Vector<Order>>,
    asks: UnorderedMap<Price, Vector<Order>>,
    positions: UnorderedMap<TraderId, Position>, // trader_id -> Position
    trades: Vector<Trade>,
    mark_price: f64,
    funding_rate: f64,
    min_margin: f64,
    max_leverage: f64,
}
#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum OrderError {
    InsufficientMargin,
    ExceedsMaxLeverage,
    InsufficientLiquidity,
    InvalidPrice,
    InvalidSize,
    OrderNotFound,
    PositionNotFound,
    LiquidationError,
}

impl Position {
    pub fn new(size: f64, entry_price: f64, leverage: f64, margin: f64) -> Self {
        let liquidation_price = if size > 0.0 {
            entry_price * (1.0 - 1.0 / leverage)
        } else {
            entry_price * (1.0 + 1.0 / leverage)
        };

        Self {
            size,
            entry_price,
            leverage,
            liquidation_price,
            unrealized_pnl: 0.0,
            margin,
        }
    }
    pub fn update_unrealized_pnl(&mut self, current_price: f64) {
        self.unrealized_pnl = if self.size > 0.0 {
            self.size * (current_price - self.entry_price)
        } else {
            self.size * (self.entry_price - current_price)
        };
    }
    pub fn should_liquidate(&self, current_price: f64) -> bool {
        if self.size > 0.0 {
            current_price <= self.liquidation_price
        } else {
            current_price >= self.liquidation_price
        }
    }
}

impl Order {
    pub fn new(
        trader_id: TraderId,
        price: f64,
        size: f64,
        side: Side,
        order_type: OrderType,
        leverage: f64,
    ) -> Self {
        Self {
            id: 0, // Will be set by the orderbook
            trader_id,
            price,
            size,
            side,
            order_type,
            leverage,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Clones all orders in the given vector.
pub fn clone_vec(v: &Vector<Order>) -> Vector<Order> {
    let mut new_vec = Vector::new();
    let iter = v.entries().unwrap();
    for order in iter {
        new_vec.push(order.clone()).unwrap();
    }
    new_vec
}

/// Retains only those elements in the vector that satisfy the predicate.
fn retain<F, T>(vec: &mut Vector<T>, mut predicate: F)
where
    T: BorshSerialize + BorshDeserialize,
    F: FnMut(&T) -> bool,
{
    let mut kept_values = Vec::new();

    // Collect elements that satisfy the predicate
    for entry in vec.entries().unwrap() {
        if predicate(&entry) {
            kept_values.push(entry);
        }
    }

    // Clear the existing vector
    vec.clear().unwrap();

    // Reinsert the kept values
    for value in kept_values {
        vec.push(value).unwrap();
    }
}

impl PerpetualOrderBook {
    pub fn new(initial_mark_price: f64, min_margin: f64, max_leverage: f64) -> Self {
        Self {
            next_id: 1,
            bids: UnorderedMap::new(),
            asks: UnorderedMap::new(),
            positions: UnorderedMap::new(),
            trades: Vector::new(),
            mark_price: initial_mark_price,
            funding_rate: 0.0,
            min_margin,
            max_leverage,
        }
    }

    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn find_best_ask(&self) -> Option<(Price, Vector<Order>)> {
        // The keys in `asks` have been created from rounded prices.
        self.asks
            .entries()
            .unwrap()
            .filter(|(_, level)| level.len().unwrap() != 0)
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(price, level)| (price, level))
    }

    fn find_best_bid(&self) -> Option<(Price, Vector<Order>)> {
        self.bids
            .entries()
            .unwrap()
            .filter(|(_, level)| level.len().unwrap() != 0)
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(price, level)| (price, level))
    }

    pub fn place_order(&mut self, mut order: Order, margin: f64) -> Result<Vec<Trade>, OrderError> {
        if order.price <= 0.0 || order.size <= 0.0 {
            return Err(OrderError::InvalidPrice);
        }

        if order.leverage > self.max_leverage {
            return Err(OrderError::ExceedsMaxLeverage);
        }

        let required_margin = (order.price * order.size) / order.leverage;
        if required_margin < self.min_margin || required_margin > margin {
            return Err(OrderError::InsufficientMargin);
        }

        // Generate a unique order ID.
        order.id = self.generate_id();
        let mut trades = Vec::new();
        let mut remaining_size = order.size;
        // Normalize the order's limit price.
        let order_limit_price = round_price(order.price, 8);

        match order.side {
            Side::Long => {
                while remaining_size > 0.0 {
                    if let Some((ask_key, ask_level)) = self.find_best_ask() {
                        // Convert the key back to f64.
                        let ask_price = f64::from_le_bytes(ask_key.clone());
                        // Compare using the normalized (rounded) price.
                        if order.order_type == OrderType::Limit && ask_price > order_limit_price {
                            break;
                        }

                        let mut updated_level = clone_vec(&ask_level);
                        let mut updated_level_entries: Vec<_> =
                            updated_level.entries().unwrap().collect();
                        for maker_order in updated_level_entries.iter_mut() {
                            if maker_order.size == 0.0 {
                                continue;
                            }

                            let trade_size = remaining_size.min(maker_order.size);
                            let trade = Trade {
                                id: self.generate_id(),
                                maker_order_id: maker_order.id,
                                taker_order_id: order.id,
                                price: ask_price,
                                size: trade_size,
                                timestamp: order.timestamp,
                            };

                            self.update_position(
                                order.trader_id,
                                trade_size,
                                ask_price,
                                order.leverage,
                                margin,
                            )?;
                            self.update_position(
                                maker_order.trader_id,
                                -trade_size,
                                ask_price,
                                maker_order.leverage,
                                margin,
                            )?;

                            maker_order.size -= trade_size;
                            remaining_size -= trade_size;
                            trades.push(trade);

                            if remaining_size == 0.0 {
                                break;
                            }
                        }

                        // Remove fully filled orders from this price level.
                        retain(&mut updated_level, |o: &Order| o.size > 0.0);
                        if updated_level.len().unwrap() == 0 {
                            self.asks.remove(&ask_key).unwrap();
                        } else {
                            self.asks
                                .insert(ask_key, clone_vec(&updated_level))
                                .unwrap();
                        }
                    } else {
                        break;
                    }
                }
            }
            Side::Short => {
                while remaining_size > 0.0 {
                    if let Some((bid_key, bid_level)) = self.find_best_bid() {
                        let bid_price = f64::from_le_bytes(bid_key.clone());
                        if order.order_type == OrderType::Limit && bid_price < order_limit_price {
                            break;
                        }

                        let mut updated_level = clone_vec(&bid_level);
                        let mut updated_level_iter: Vec<_> =
                            updated_level.entries().unwrap().collect();
                        for maker_order in updated_level_iter.iter_mut() {
                            if maker_order.size == 0.0 {
                                continue;
                            }

                            let trade_size = remaining_size.min(maker_order.size);
                            let trade = Trade {
                                id: self.generate_id(),
                                maker_order_id: maker_order.id,
                                taker_order_id: order.id,
                                price: bid_price,
                                size: trade_size,
                                timestamp: order.timestamp,
                            };

                            self.update_position(
                                order.trader_id,
                                -trade_size,
                                bid_price,
                                order.leverage,
                                margin,
                            )?;
                            self.update_position(
                                maker_order.trader_id,
                                trade_size,
                                bid_price,
                                maker_order.leverage,
                                margin,
                            )?;

                            maker_order.size -= trade_size;
                            remaining_size -= trade_size;
                            trades.push(trade);

                            if remaining_size == 0.0 {
                                break;
                            }
                        }

                        retain(&mut updated_level, |o| o.size > 0.0);
                        if updated_level.len().unwrap() == 0 {
                            self.bids.remove(&bid_key).unwrap();
                        } else {
                            self.bids
                                .insert(bid_key, clone_vec(&updated_level))
                                .unwrap();
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // If this is a limit order and there is remaining size, add it to the order book.
        if remaining_size > 0.0 && order.order_type == OrderType::Limit {
            order.size = remaining_size;
            // Normalize the price before converting to bytes.
            let rounded_price = round_price(order.price, 8);
            let key = rounded_price.to_le_bytes();
            match order.side {
                Side::Long => {
                    let mut level = self
                        .bids
                        .get(&key)
                        .unwrap_or(Some(Vector::<Order>::new()))
                        .unwrap();
                    level.push(order.clone()).unwrap();
                    self.bids.insert(key, clone_vec(&level)).unwrap();
                }
                Side::Short => {
                    let mut level = self.asks.get(&key).unwrap_or(Some(Vector::new())).unwrap();
                    level.push(order.clone()).unwrap();
                    self.asks.insert(key, clone_vec(&level)).unwrap();
                }
            };
        }

        Ok(trades)
    }

    fn update_position(
        &mut self,
        trader_id: TraderId,
        size_delta: f64,
        price: f64,
        leverage: f64,
        margin: f64,
    ) -> Result<(), OrderError> {
        let mut position = self
            .positions
            .get(&trader_id)
            .unwrap_or(Some(Position::new(0.0, price, leverage, margin)))
            .unwrap();

        let new_size = position.size + size_delta;

        if new_size.abs() < position.size.abs() {
            position.size = new_size;
            position.update_unrealized_pnl(self.mark_price);
            self.positions.insert(trader_id, position).unwrap();
            return Ok(());
        }

        let weighted_price = if position.size == 0.0 {
            price
        } else {
            (position.entry_price * position.size.abs() + price * size_delta.abs())
                / (position.size.abs() + size_delta.abs())
        };

        position.size = new_size;
        position.entry_price = weighted_price;
        position.leverage = leverage;
        position.margin = margin;

        position.liquidation_price = if new_size > 0.0 {
            weighted_price * (1.0 - 1.0 / leverage)
        } else {
            weighted_price * (1.0 + 1.0 / leverage)
        };

        position.update_unrealized_pnl(self.mark_price);
        self.positions.insert(trader_id, position).unwrap();

        Ok(())
    }

    pub fn update_mark_price(&mut self, new_mark_price: f64) -> Vec<u64> {
        self.mark_price = new_mark_price;
        let mut liquidated_traders = Vec::new();

        for (trader_id, mut position) in self.positions.entries().unwrap() {
            position.update_unrealized_pnl(new_mark_price);
            if position.should_liquidate(new_mark_price) {
                liquidated_traders.push(u64::from_le_bytes(trader_id));
            }
        }

        for trader_id in &liquidated_traders {
            self.positions.remove(&trader_id.to_le_bytes()).unwrap();
        }

        liquidated_traders
    }

    pub fn update_funding_rate(&mut self) {
        let mut total_longs = 0.0;
        let mut total_shorts = 0.0;

        for (_, position) in self.positions.entries().unwrap() {
            if position.size > 0.0 {
                total_longs += position.size;
            } else {
                total_shorts += position.size.abs();
            }
        }

        if total_longs + total_shorts > 0.0 {
            self.funding_rate = (total_longs - total_shorts) / (total_longs + total_shorts) * 0.01;
        }

        let entries: Vec<_> = self.positions.entries().unwrap().collect();
        for (trader_id, position) in entries {
            let mut new_position = position.clone();
            let funding_payment = position.size * self.mark_price * self.funding_rate;
            new_position.margin -= funding_payment;
            self.positions.insert(trader_id, new_position).unwrap();
        }
    }
}

#[app::event]
pub enum Event {
    ProposalCreated { id: ProposalId },
    ApprovedProposal { id: ProposalId },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct CreateProposalRequest {
    pub action_type: String,
    pub params: serde_json::Value,
}

#[app::logic]
impl AppState {
    #[app::init]
    pub fn init() -> AppState {
        AppState {
            messages: UnorderedMap::new(),
            perpetual_order_book: UnorderedMap::new(),
        }
    }

    pub fn create_new_proposal(
        &mut self,
        request: CreateProposalRequest,
    ) -> Result<ProposalId, Error> {
        env::log("Starting create_new_proposal");
        env::log(&format!("Request type: {}", request.action_type));

        let proposal_id = match request.action_type.as_str() {
            "ExternalFunctionCall" => {
                env::log("Processing ExternalFunctionCall");
                let receiver_id = request.params["receiver_id"]
                    .as_str()
                    .ok_or_else(|| Error::msg("receiver_id is required"))?;
                let method_name = request.params["method_name"]
                    .as_str()
                    .ok_or_else(|| Error::msg("method_name is required"))?;
                let args = request.params["args"]
                    .as_str()
                    .ok_or_else(|| Error::msg("args is required"))?;
                let deposit = request.params["deposit"]
                    .as_str()
                    .ok_or_else(|| Error::msg("deposit is required"))?
                    .parse::<u128>()?;

                env::log(&format!(
                    "Parsed values: receiver_id={}, method_name={}, args={}, deposit={}",
                    receiver_id, method_name, args, deposit
                ));

                Self::external()
                    .propose()
                    .external_function_call(
                        receiver_id.to_string(),
                        method_name.to_string(),
                        args.to_string(),
                        deposit,
                    )
                    .send()
            }
            "Transfer" => {
                env::log("Processing Transfer");
                let receiver_id = request.params["receiver_id"]
                    .as_str()
                    .ok_or_else(|| Error::msg("receiver_id is required"))?;
                let amount = request.params["amount"]
                    .as_str()
                    .ok_or_else(|| Error::msg("amount is required"))?
                    .parse::<u128>()?;

                Self::external()
                    .propose()
                    .transfer(AccountId(receiver_id.to_string()), amount)
                    .send()
            }
            "SetContextValue" => {
                env::log("Processing SetContextValue");
                let key = request.params["key"]
                    .as_str()
                    .ok_or_else(|| Error::msg("key is required"))?
                    .as_bytes()
                    .to_vec()
                    .into_boxed_slice();
                let value = request.params["value"]
                    .as_str()
                    .ok_or_else(|| Error::msg("value is required"))?
                    .as_bytes()
                    .to_vec()
                    .into_boxed_slice();

                Self::external()
                    .propose()
                    .set_context_value(key, value)
                    .send()
            }
            "SetNumApprovals" => Self::external()
                .propose()
                .set_num_approvals(
                    request.params["num_approvals"]
                        .as_u64()
                        .ok_or(Error::msg("num_approvals is required"))? as u32,
                )
                .send(),
            "SetActiveProposalsLimit" => Self::external()
                .propose()
                .set_active_proposals_limit(
                    request.params["active_proposals_limit"]
                        .as_u64()
                        .ok_or(Error::msg("active_proposals_limit is required"))?
                        as u32,
                )
                .send(),
            "DeleteProposal" => Self::external()
                .propose()
                .delete(ProposalId(
                    hex::decode(
                        request.params["proposal_id"]
                            .as_str()
                            .ok_or_else(|| Error::msg("proposal_id is required"))?,
                    )?
                    .try_into()
                    .map_err(|_| Error::msg("Invalid proposal ID length"))?,
                ))
                .send(),
            _ => return Err(Error::msg("Invalid action type")),
        };

        env::emit(&Event::ProposalCreated { id: proposal_id });

        let old = self.messages.insert(proposal_id, Vector::new())?;
        if old.is_some() {
            return Err(Error::msg("proposal already exists"));
        }

        Ok(proposal_id)
    }

    pub fn create_order() {}

    pub fn approve_proposal(&self, proposal_id: ProposalId) -> Result<(), Error> {
        // fixme: should we need to check this?
        // self.messages
        //     .get(&proposal_id)?
        //     .ok_or(Error::msg("proposal not found"))?;

        Self::external().approve(proposal_id);

        env::emit(&Event::ApprovedProposal { id: proposal_id });

        Ok(())
    }

    pub fn get_proposal_messages(&self, proposal_id: ProposalId) -> Result<Vec<Message>, Error> {
        let Some(msgs) = self.messages.get(&proposal_id)? else {
            return Ok(vec![]);
        };

        let entries = msgs.entries()?;

        Ok(entries.collect())
    }

    pub fn send_proposal_messages(
        &mut self,
        proposal_id: ProposalId,
        message: Message,
    ) -> Result<(), Error> {
        let mut messages = self.messages.get(&proposal_id)?.unwrap_or_default();

        messages.push(message)?;

        self.messages.insert(proposal_id, messages)?;

        Ok(())
    }

    pub fn create_perpetual_order_book(
        &mut self,
        token_id: TokenId,
        initial_mark_price: f64,
        min_margin: f64,
        max_leverage: f64,
    ) -> Result<(), Error> {
        let order_book = PerpetualOrderBook::new(initial_mark_price, min_margin, max_leverage);
        self.perpetual_order_book.insert(token_id, order_book)?;
        Ok(())
    }

    pub fn place_order(
        &mut self,
        token_id: TokenId,
        order: Order,
        margin: f64,
    ) -> Result<Vec<Trade>, OrderError> {
        let mut order_book = self
            .perpetual_order_book
            .get(&token_id)
            .map_err(|_| OrderError::OrderNotFound)?
            .ok_or(OrderError::OrderNotFound)?;
        let trades = order_book.place_order(order, margin)?;
        self.perpetual_order_book
            .insert(token_id, order_book)
            .map_err(|_| OrderError::OrderNotFound)?;
        Ok(trades)
    }

    pub fn update_mark_price(
        &mut self,
        token_id: TokenId,
        new_mark_price: f64,
    ) -> Result<Vec<u64>, OrderError> {
        let mut order_book = self
            .perpetual_order_book
            .get(&token_id)
            .map_err(|_| OrderError::OrderNotFound)?
            .ok_or(OrderError::OrderNotFound)?;
        let liquidated_traders = order_book.update_mark_price(new_mark_price);
        self.perpetual_order_book
            .insert(token_id, order_book)
            .map_err(|_| OrderError::OrderNotFound)?;
        Ok(liquidated_traders)
    }

    pub fn update_funding_rate(&mut self, token_id: TokenId) -> Result<(), OrderError> {
        let mut order_book = self
            .perpetual_order_book
            .get(&token_id)
            .map_err(|_| OrderError::InvalidPrice)?
            .ok_or(OrderError::OrderNotFound)?;
        order_book.update_funding_rate();
        self.perpetual_order_book
            .insert(token_id, order_book)
            .map_err(|_| OrderError::InvalidPrice)?;
        Ok(())
    }
}
