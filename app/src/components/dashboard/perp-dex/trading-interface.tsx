import { Bitcoin, DollarSign, Info } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Slider } from "@/components/ui/slider";
import { Button } from "@/components/ui/button";
import { useDashboardStore } from "@/lib/stores/dashboard-store";
import { useState } from "react";

export default function TradingInterface() {
  const { currencies, selectedCurrencyIndex } = useDashboardStore();
  const currentCurrency = currencies[selectedCurrencyIndex];

  const [position, setPosition] = useState<"long" | "short">("long");
  const [price, setPrice] = useState("97001.04");
  const [orderType, setOrderType] = useState("market");
  const [cryptoAmount, setCryptoAmount] = useState("");
  const [usdAmount, setUsdAmount] = useState("");
  const [buyingPowerPercentage, setBuyingPowerPercentage] = useState(0);

  return (
    <Card className="bg-card border-border">
      <CardContent className="p-4">
        <Tabs
          value={position}
          onValueChange={(val) => setPosition(val as "long" | "short")}
          className="w-full"
        >
          <TabsList className="w-full mb-4">
            <TabsTrigger value="long" className="flex-1">
              Long
            </TabsTrigger>
            <TabsTrigger value="short" className="flex-1">
              Short
            </TabsTrigger>
          </TabsList>
        </Tabs>

        <div className="space-y-4">
          {/* Price & Order Type */}
          <div>
            <div className="flex justify-between mb-2">
              <label className="text-sm text-muted-foreground">Price</label>
              <label className="text-sm text-muted-foreground">
                Order Type
              </label>
            </div>
            <div className="flex gap-2">
              <input
                type="text"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                className="flex-1 bg-background border border-border rounded-md px-3 py-2"
              />
              <Select value={orderType} onValueChange={setOrderType}>
                <SelectTrigger className="w-[140px] bg-background border-border">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="market">Market</SelectItem>
                  <SelectItem value="limit">Limit</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          {/* Amount Input */}
          <div>
            <label className="text-sm text-muted-foreground mb-2 block">
              Amount
            </label>
            <div className="space-y-2">
              <div className="flex items-center gap-2 bg-background border border-border rounded-md px-3 py-2">
                <img
                  src={currentCurrency.iconPath}
                  alt={currentCurrency.symbol}
                  className="w-4 h-4"
                />
                <input
                  type="text"
                  placeholder={currentCurrency.symbol}
                  value={cryptoAmount}
                  onChange={(e) => setCryptoAmount(e.target.value)}
                  className="bg-transparent flex-1 outline-none"
                />
              </div>
              <div className="flex items-center gap-2 bg-background border border-border rounded-md px-3 py-2">
                <DollarSign className="w-4 h-4" />
                <input
                  type="text"
                  placeholder="USD"
                  value={usdAmount}
                  onChange={(e) => setUsdAmount(e.target.value)}
                  className="bg-transparent flex-1"
                />
              </div>
            </div>
          </div>

          {/* Buying Power */}
          <div>
            <div className="flex items-center gap-1 mb-2">
              <label className="text-sm text-muted-foreground">
                Buying Power
              </label>
              <Info className="w-4 h-4 text-muted-foreground" />
              <span className="ml-auto text-sm">≈ $-</span>
            </div>
            <Slider
              value={[buyingPowerPercentage]}
              onValueChange={([value]) => setBuyingPowerPercentage(value)}
              max={100}
              step={1}
              className="my-4"
            />
            <div className="text-sm text-muted-foreground">
              {buyingPowerPercentage}% of BP
            </div>
          </div>
    
          <Button variant={position === "long" ? "default" : "destructive"} className="w-full">
            Confirm {position.charAt(0).toUpperCase() + position.slice(1)}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
