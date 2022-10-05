import default_image from "../assets/default.jpg";
import { Link } from "react-router-dom";

const Service = ({ id, title, duration, price }) => {
  return (
    <div className="w-1/4 rounded bg-gray-200 border hover:border-indigo-400">
      <Link to={"/services/" + id}>
        <img src={default_image} className="w-full rounded-t h-24" />
        <div className="p-2 flex items-center justify-between">
          <div className="place-self-start">
            <p>{title}</p>
          </div>
          <div className="flex flex-col">
            <p className="text-right">{duration}</p>
            <p className="text-right">{price}₴</p>
          </div>
        </div>
      </Link>
    </div>
  );
};

export default Service;
