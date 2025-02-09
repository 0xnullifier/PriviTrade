import { Card, CardContent, CardHeader } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

export default function MarketTrades() {
  const trades = [
    { size: "0.00103012", time: "2/10 05:59", price: "97,075.4", type: "sell" },
    { size: "0.00102960", time: "2/10 05:56", price: "97,125.0", type: "sell" },
    { size: "0.00154341", time: "2/10 05:55", price: "97,187.1", type: "sell" },
    { size: "0.00102778", time: "2/10 05:48", price: "97,199.5", type: "buy" },
    { size: "0.00102830", time: "2/10 05:36", price: "97,149.9", type: "buy" },
  ];

  return (
    <Card className="bg-card border-border">
      <CardHeader className="flex flex-row items-center justify-between py-3">
        <div className="font-semibold">Market Trades</div>
        <span className="text-sm text-muted-foreground">24H</span>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="text-muted-foreground">SIZE</TableHead>
              <TableHead className="text-muted-foreground">TIME</TableHead>
              <TableHead className="text-right text-muted-foreground">
                PRICE
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {trades.map((trade, i) => (
              <TableRow key={i}>
                <TableCell
                  className={
                    trade.type === "sell" ? "text-red-400" : "text-green-400"
                  }
                >
                  {trade.size}
                </TableCell>
                <TableCell className="text-muted-foreground">
                  {trade.time}
                </TableCell>
                <TableCell className="text-right">{trade.price}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
