import Overview from "./overview";
import TradingChart from "./trading-chart";
import TradingInterface from "./trading-interface";
import CalimeroNodes from "./calimero-nodes";
import MarketTrades from "./market-trades";
import { Button } from "@/components/ui/button";

const PerpDex = () => {
  return (
    <div className="p-6 space-y-4">
      {/* Top Overview Bar */}
      <Overview />

      {/* Main Grid Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Left: Trading Chart */}
        <div className="lg:col-span-2">
          <TradingChart />
        </div>

        {/* Right: Trading Interface */}
        <div >
          <TradingInterface />
        </div>
      </div>

      {/* Bottom Grid Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Left: Connect Wallet */}
        <div className="lg:col-span-2 flex items-center justify-center bg-card border border-border rounded-md p-6">
          <Button className="bg-blue-500 hover:bg-blue-600">
            🔗 Connect Wallet
          </Button>
        </div>

        {/* Right: Market Trades */}
        <div>
          <MarketTrades />
        </div>
      </div>
    </div>
  );
};

export default PerpDex;
