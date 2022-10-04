import logo from "../assets/logo.png";

const Header = () => {
  return (
    <div className="w-full h-12 flex justify-between items-center bg-gray-800">
      <a href="" className="h-3/4 w-10 mb-1">
        <img src={logo} alt="logo" className="h-full mx-4" />
      </a>
      <a href="" className="mx-4 text-indigo-300 rounded p-1 hover:bg-gray-700">
        log in
      </a>
    </div>
  );
};

export default Header;
