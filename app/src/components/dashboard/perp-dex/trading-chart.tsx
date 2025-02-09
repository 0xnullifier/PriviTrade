import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useDashboardStore } from "@/lib/stores/dashboard-store";
import { useEffect, useRef } from "react";

export default function TradingChart() {
  const container = useRef(null);
  const { currencies, selectedCurrencyIndex } = useDashboardStore();
  useEffect(() => {
    const currentContainer = container.current;
    if (!currentContainer) return;
    currentContainer.innerHTML = "";

    const script = document.createElement("script");
    script.src =
      "https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js";
    script.type = "text/javascript";
    script.async = true;
    script.innerHTML = `
            {
              "autosize": true,
              "symbol": "${currencies[selectedCurrencyIndex].symbol}",
              "interval": "D", 
              "timezone": "Etc/UTC",
              "theme": "dark",
              "style": "1",
              "locale": "en",
              "allow_symbol_change": true,
              "calendar": false,
              "support_host": "https://www.tradingview.com"
            }`;

    currentContainer.appendChild(script);

    return () => {
      if (currentContainer) currentContainer.innerHTML = "";
    };
  }, [selectedCurrencyIndex, currencies]);

  return (
    <Card className="bg-card border-border">
      <CardHeader className="border-b border-border p-0">
        <div className="flex items-center gap-4">
          <Tabs defaultValue="price">
            <TabsList className="bg-background">
              <TabsTrigger value="price">Price</TabsTrigger>
              <TabsTrigger value="funding">Funding</TabsTrigger>
            </TabsList>
          </Tabs>
        </div>
      </CardHeader>
      <CardContent className="flex items-center justify-center text-muted-foreground overflow-scroll p-0 h-[400px] rounded-lg">
        <div
          className="tradingview-widget-container w-full h-full"
          ref={container}
        >
          {" "}
          <div
            className="tradingview-widget-container__widget"
            style={{ width: "100%" }}
          ></div>
        </div>
      </CardContent>
    </Card>
  );
}
