import { useDashboardStore } from "@/lib/stores/dashboard-store";
import { useEffect, useState } from "react";

interface PriceData {
  price: number;
  priceChange: number;
  markPrice: number;
  indexPrice: number;
  volume24h: number;
}

const Overview = () => {
  const { currencies, selectedCurrencyIndex } = useDashboardStore();
  const [priceData, setPriceData] = useState<PriceData | null>(null);

  useEffect(() => {
    if (!currencies || currencies.length === 0) return;

    const fetchPriceData = async () => {
      try {
        // Simulated API call - replace with actual API endpoint
        const response = await fetch(
          `/api/price/${currencies[selectedCurrencyIndex].symbol}`
        );
        const data = await response.json();
        setPriceData(data);
      } catch (error) {
        console.error("Failed to fetch price data:", error);
        // Populate with sample data if API fails
        setPriceData({
          price: 47382.51,
          priceChange: 2.34,
          markPrice: 47385.12,
          indexPrice: 47380.98,
          volume24h: 1234567890
        });
      }
    };

    fetchPriceData();
    const interval = setInterval(fetchPriceData, 5000); // Update every 5 seconds

    return () => clearInterval(interval);
  }, [currencies, selectedCurrencyIndex]);

  // Initialize with sample data if no data exists
  if (!priceData) {
    setPriceData({
      price: 47382.51,
      priceChange: 2.34,
      markPrice: 47385.12,
      indexPrice: 47380.98,
      volume24h: 1234567890
    });
    return null;
  }

  if (!currencies || currencies.length === 0) {
    return null;
  }

  const selectedCurrency = currencies[selectedCurrencyIndex];
  const priceChangeColor =
    priceData.priceChange >= 0 ? "text-green-400" : "text-red-400";

  return (
    <div className="flex items-center gap-8 mb-6">
      {/* Currency Details */}
      <div className="flex items-center gap-2">
        <img
          src={selectedCurrency.iconPath}
          alt={selectedCurrency.symbol}
          className="w-8 h-8"
        />
        <div>
          <div className="flex items-center gap-2">
            <span className="font-bold">{selectedCurrency.symbol}</span>
            <span className="text-muted-foreground">
              {selectedCurrency.name}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xl font-bold">
              {priceData.price.toLocaleString()}
            </span>
            <span className={priceChangeColor}>
              {priceData.priceChange > 0 ? "+" : ""}
              {priceData.priceChange.toFixed(2)}%
            </span>
          </div>
        </div>
      </div>

      {/* Market Details */}
      <div className="grid grid-cols-3 gap-8">
        {[
          { label: "Mark Price", value: priceData.markPrice.toLocaleString() },
          {
            label: "Index Price",
            value: priceData.indexPrice.toLocaleString(),
          },
          {
            label: "Volume (24h)",
            value: priceData.volume24h.toLocaleString(),
          },
        ].map(({ label, value }, i) => (
          <div key={i}>
            <div className="text-sm text-muted-foreground">{label}</div>
            <div>{value}</div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Overview;
