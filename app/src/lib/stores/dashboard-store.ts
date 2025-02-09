import { create } from "zustand";
import { Currency } from "@/lib/types";
import { CURRENCIES } from "@/constants/currencies";

interface DashboardStoreStates {
  currencies: Currency[];
  selectedCurrencyIndex: number;
  setSelectedCurrencyIndex: (index: number) => void;
}

export const useDashboardStore = create<DashboardStoreStates>((set) => ({
  currencies: CURRENCIES,
  selectedCurrencyIndex: 0,
  setSelectedCurrencyIndex: (index: number) =>
    set({ selectedCurrencyIndex: index }),
}));
