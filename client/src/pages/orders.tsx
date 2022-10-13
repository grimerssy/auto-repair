import { useState, useEffect } from "react";
import { getAllOrders } from "../api/api.ts";
import { Typography } from "@mui/material";

const Orders = () => {
  const [orders, setOrders] = useState([]);

  useEffect(() => {
    getAllOrders().then((data) => {
      setOrders(data);
    });
  }, []);

  return (
    <div className="flex flex-col justify-center items-center">
      {orders.map((o, i) => (
        <div id={o.id} className="rounded bg-gray-200 p-10 my-8">
          <Typography sx={{ mb: 2 }} variant="h5" style={{ fontWeight: 600 }}>
            {o.service.title}
          </Typography>
          <div className="ml-10 my-2 flex flex-col justify-center">
            <Typography align="left">Start time: {o.startTime}</Typography>
            <Typography align="left">Duration: {o.service.duration}</Typography>
          </div>
          <Typography variant="h6">Contacts</Typography>
          <div className="ml-10 my-2 flex flex-col justify-center">
            <Typography align="left">{o.contact.phoneNumber}</Typography>
            {o.contact.email ? (
              <Typography align="left">{o.contact.email}</Typography>
            ) : null}
          </div>
          <Typography sx={{ mb: 2 }} variant="h6">
            Car
          </Typography>
          <div className="ml-10 flex flex-col justify-center">
            <Typography align="left">{o.carMake}</Typography>
            <Typography align="left">{o.carModel}</Typography>
            <Typography align="left">{o.carYear}</Typography>
          </div>
        </div>
      ))}
    </div>
  );
};

export default Orders;
