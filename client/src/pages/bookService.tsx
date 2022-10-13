import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService } from "../api/api.ts";
import BookNoAuth from "../components/bookNoAuth.tsx";
import { Typography } from "@mui/material";

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
            <Typography variant="h5" style={{ fontWeight: 600 }}>
              {service.title}
            </Typography>
            <Typography>price: {service.price}₴</Typography>
            <Typography>duration: {service.duration}</Typography>
          </div>
          <BookNoAuth serviceId={service.id} />
        </div>
      </div>
    </div>
  );
};

export default BookService;
