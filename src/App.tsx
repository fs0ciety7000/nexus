import Navbar from "./components/layout/Navbar";
import BentoGrid from "./components/layout/BentoGrid";
import SystemSentinel from "./components/modules/SystemSentinel";
import ObsidianViewer from "./components/modules/ObsidianViewer";
import ProtonReactor from "./components/modules/ProtonReactor";
import "./styles/globals.css";

function App() {
  return (
    <div className="min-h-screen bg-nexus-bg text-foreground">
      <Navbar />
      <BentoGrid>
        <SystemSentinel />
        <ObsidianViewer />
        <ProtonReactor />
      </BentoGrid>
    </div>
  );
}

export default App;
