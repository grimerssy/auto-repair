import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService, getOrdersByServiceId } from "../api/api.ts";
import {
  Typography,
  Paper,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";

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
        {!service.id ? (
          <Typography variant="h5">Invalid service id</Typography>
        ) : (
          <div>
            <div className="flex flex-row justify-around items-center">
              <Typography variant="h6" style={{ fontWeight: 600 }}>
                {service.title}
              </Typography>
              <Typography variant="h6">{service.duration}</Typography>
              <Typography variant="h6">{service.price}₴</Typography>
            </div>
            {orders.length === 0 ? (
              <Typography variant="h5">No such orders at the moment</Typography>
            ) : (
              <div>
                <Typography variant="h5" sx={{ my: 6 }}>
                  Orders
                </Typography>
                <TableContainer component={Paper}>
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell>Start time</TableCell>
                        <TableCell>Phone number</TableCell>
                        <TableCell>Email</TableCell>
                        <TableCell>Car make</TableCell>
                        <TableCell>Car model</TableCell>
                        <TableCell>Car year</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {orders.map((o, i) => (
                        <TableRow>
                          <TableCell>{o.startTime}</TableCell>
                          <TableCell>{o.contact.phoneNumber}</TableCell>
                          <TableCell>{o.contact.email}</TableCell>
                          <TableCell>{o.carMake}</TableCell>
                          <TableCell>{o.carModel}</TableCell>
                          <TableCell>{o.carYear}</TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </TableContainer>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default OrdersByService;
