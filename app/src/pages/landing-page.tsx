import { FlickeringGrid } from "@/components/magicui/flickering-grid";
import { Button } from "@/components/ui/button";
import { ChevronRight } from "lucide-react";
import { useNavigate } from "react-router-dom";

const LandingPage = () => {
  const navigate = useNavigate();
  return (
    <div className="w-full h-screen flex flex-col gap-4 items-center justify-center">
      <div className="absolute h-screen w-full overflow-hidden rounded-lg border bg-background -z-10">
        <FlickeringGrid
          className="relative inset-0 z-0 [mask-image:radial-gradient(400px_circle_at_center,white,transparent)]"
          squareSize={4}
          gridGap={6}
          color="#FFFFFF"
          maxOpacity={0.3}
          flickerChance={0.1}
        />
      </div>
      <h1 className="text-8xl font-bold tracking-tight">PriviTrade</h1>
      <p className="text-2xl tracking-tight font-medium text-muted-foreground">
        Perpetual trading, with privacy
      </p>
      <Button
        className="font-medium tracking-tight mt-4"
        size={"lg"}
        effect={"expandIcon"}
        iconPlacement="right"
        icon={ChevronRight}
        onClick={() => navigate("/auth")}
      >
        Login
      </Button>
    </div>
  );
};

export default LandingPage;
