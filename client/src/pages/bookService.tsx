import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService } from "../api/api.ts";
import BookNoAuth from "../components/bookNoAuth.tsx";

const BookService = () => {
  const { id } = useParams();
  const [service, setService] = useState({});

  useEffect(() => {
    getService(id).then((data) => setService(data));
  }, []);

  return (
    <div className="flex flex-col justify-center items-center my-10">
      <div className="flex flex-col justify-center items-center bg-gray-200 px-10 py-4 rounded">
        <div className="flex flex-col justify-center items-center">
          <div className="flex flex-col justify-center items-center">
            <p className="text-lg font-semibold mb-1">{service.title}</p>
            <p className="mb-1">price: {service.price}₴</p>
            <p className="mb-1">duration: {service.duration}</p>
          </div>
          <BookNoAuth serviceId={service.id} />
        </div>
      </div>
    </div>
  );
};

export default BookService;
