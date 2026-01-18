import { Button } from "@/components/ui/button";
import appIcon from "../../assets/icon2.png";

export function WelcomeScreen({ onGetStarted }: { onGetStarted: () => void }) {
  return (
    <div className="flex min-h-screen">
      {/* Left Side - Branding */}
      <div className="flex flex-1 flex-col items-center justify-center bg-primary/5 px-12 py-8">
        <div className="flex flex-col items-center max-w-lg">
          <img src={appIcon} alt="YOLO NDJSON Converter" className="mb-8 h-24 w-24" />
          <h1 className="mb-4 text-4xl font-bold tracking-tight text-foreground text-center">
            YOLO NDJSON
            <br />
            <span className="text-primary">to ZIP</span>
          </h1>
          <p className="mb-8 text-lg text-muted-foreground text-center">
            Convert your NDJSON annotation exports to popular ML formats.
            Fast, private, and runs entirely on your machine.
          </p>
        </div>
      </div>

      {/* Right Side - Get Started */}
      <div className="flex flex-1 flex-col justify-center px-12 py-8">
        <div className="max-w-md">
          <Button
            onClick={onGetStarted}
            size="lg"
            className="mt-4 w-full py-6 text-lg"
          >
            Get Started
          </Button>
        </div>
      </div>
    </div>
  );
}
