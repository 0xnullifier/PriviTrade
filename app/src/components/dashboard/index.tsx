import PerpDex from "./perp-dex";
import SideBar from "./sidebar";

const Dashboard = () => {
  return (
    <div className="flex flex-row w-full h-screen overflow-y-auto">
      <SideBar />
      <div className="flex-1 overflow-y-auto">
        <PerpDex />
      </div>
    </div>
  );
};

export default Dashboard;
