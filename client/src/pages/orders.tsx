import { useState, useEffect } from "react";
import { getAllOrders } from "../api/api.ts";

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
          <p className="text-lg font-semibold">{o.service.title}</p>
          <div className="ml-10 flex flex-col justify-center">
            <p className="text-left">Start time: {o.startTime}</p>
            <p className="text-left">Duration: {o.service.duration}</p>
          </div>
          <p className="text-lg mt-6">Contacts</p>
          <div className="ml-10 flex flex-col justify-center">
            <p className="text-left">{o.contact.phoneNumber}</p>
            {o.contact.email ? (
              <p className="text-left">{o.contact.email}</p>
            ) : null}
          </div>
          <p className="text-lg mt-6">Car</p>
          <div className="ml-10 flex flex-col justify-center">
            <p className="text-left">{o.carMake}</p>
            <p className="text-left">{o.carModel}</p>
            <p className="text-left">{o.carYear}</p>
          </div>
        </div>
      ))}
    </div>
  );
};

export default Orders;
