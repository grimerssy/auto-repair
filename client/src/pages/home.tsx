import { useState, useEffect } from "react";
import Service from "../components/service.tsx";
import { getAllServices } from "../api/api.ts";

const Home = () => {
  const [services, setServices] = useState([]);

  useEffect(() => {
    getAllServices().then((data) => {
      let nested = [];
      for (let i = 0; i < Math.ceil(data.length / 3); i++) {
        nested.push([]);
      }
      for (let i = 0; i < data.length; i++) {
        let index = Math.floor(i / 3);
        nested[index].push(data[i]);
      }
      setServices(nested);
    });
  }, []);

  return (
    <div className="flex flex-col mx-16 items-center">
      {services.map((n, i) => (
        <div class="w-11/12 flex flex-row justify-between items-center mt-10">
          {n.map((s, i) => (
            <Service
              id={s.id}
              title={s.title}
              price={s.price}
              duration={s.duration}
            />
          ))}
        </div>
      ))}
    </div>
  );
};

export default Home;
