import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService, getOrdersByServiceId } from "../api/api.ts";

const OrdersByService = () => {
  const { id } = useParams();
  const [service, setService] = useState({});
  const [orders, setOrders] = useState([]);

  useEffect(() => {
    getService(id).then((data) => {
      setService(data);
    });
  }, []);
  useEffect(() => {
    getOrdersByServiceId(id).then((data) => {
      setOrders(data);
    });
  }, []);

  return (
    <div className="flex flex-col justify-center items-center p-10 rounded bg-gray-200 w-3/4 mt-10 mx-auto">
      <div className="flex flex-row justify-around items-center w-3/4 my-4">
        <p className="text-center font-semibold">{service.title}</p>
        <p className="text-center">{service.duration}</p>
        <p className="text-center">{service.price}</p>
      </div>
      {orders.length === 0 ? (
        <p>No such orders at the moment</p>
      ) : (
        <div>
          <p className="text-lg my-6 text-center">Orders</p>
          <table>
            <tr className="py-8">
              <td className="border border-gray-800 px-4 text-center text-lg">
                Start time
              </td>
              <td className="border border-gray-800 px-4 text-center text-lg">
                Phone number
              </td>
              <td className="border border-gray-800 px-4 text-center text-lg">
                Email
              </td>
              <td className="border border-gray-800 px-4 text-center text-lg">
                Car make
              </td>
              <td className="border border-gray-800 px-4 text-center text-lg">
                Car model
              </td>
              <td className="border border-gray-800 px-4 text-center text-lg">
                Car year
              </td>
            </tr>
            {orders.map((o, i) => (
              <tr key={o.id} className="py-8 border">
                <td className="border border-gray-800 px-4">{o.startTime}</td>
                <td className="border border-gray-800 px-4">
                  {o.contact.phoneNumber}
                </td>
                <td className="border border-gray-800 px-4">
                  {o.contact.email}
                </td>
                <td className="border border-gray-800 px-4">{o.carMake}</td>
                <td className="border border-gray-800 px-4">{o.carModel}</td>
                <td className="border border-gray-800 px-4">{o.carYear}</td>
              </tr>
            ))}
          </table>
        </div>
      )}
    </div>
  );
};

export default OrdersByService;
