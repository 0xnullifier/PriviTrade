import { useDashboardStore } from "@/lib/stores/dashboard-store";
import { cn } from "@/lib/utils";

const SideBar = () => {
  const { currencies, selectedCurrencyIndex, setSelectedCurrencyIndex } =
    useDashboardStore();
  return (
    <div
      className={cn(
        "w-56",
        "bg-muted/40",
        "border-r p-3",
        "flex flex-col items-center gap-2"
      )}
    >
      <p
        className={cn(
          "text-lg font-bold tracking-tight",
          "text-transparent bg-clip-text",
          "bg-gradient-to-t from-muted-foreground to-foreground"
        )}
      >
        PriviTrade
      </p>
      <div
        className={cn(
          "w-full py-2",
          "bg-secondary text-foreground/60",
          "text-sm text-center font-medium tracking-tight",
          "rounded-lg border-t border-t-muted-foreground/10 mb-4"
        )}
      >
        Welcome 👋
      </div>
      {/* <div className="bg-muted h-[1px] w-full my-2" /> */}
      <div className="w-full">
        <p className="text-xs text-muted-foreground tracking-tight font-medium">Trades</p>
        <div className="flex flex-col gap-2 mt-2">
          {currencies.map((currency, index) => (
            <button
              key={currency.name}
              className={cn(
                "inline-flex items-center gap-2",
                "w-full p-2",
                "text-sm text-center font-medium tracking-tight",
                "rounded-lg",
                "focus:outline-none",
                "transition-colors",
                "hover:bg-secondary",
                index === selectedCurrencyIndex
                  ? "bg-secondary text-foreground border-b border-b-muted-foreground/10"
                  : "bg-transparent text-muted-foreground"
              )}
              onClick={() => setSelectedCurrencyIndex(index)}
            >
              <img src={currency.iconPath} alt={currency.toString()} className="w-5 h-5" />
              <span>{currency.symbol}</span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};

export default SideBar;
