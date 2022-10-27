import { Link } from "react-router-dom";
import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import {
  getService,
  getOrdersByServiceId,
  deleteOrderById,
} from "../../../api/api";
import {
  Box,
  Grid,
  Typography,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";
import { Order, Service } from "models";

const OrdersByService = () => {
  const { id } = useParams();
  const [service, setService] = useState<Service>({
    id: "",
    title: "",
    price: 0,
    duration: "",
  });
  const [orders, setOrders] = useState<Order[]>([]);

  useEffect(() => {
    getService(id || "").then((data) => {
      setService(data);
    });
  }, []);
  useEffect(() => {
    getOrdersByServiceId(id || "").then((data) => {
      setOrders(data);
    });
  }, []);

  return (
    <Box sx={{ mt: 4, display: "flex", justifyContent: "center" }}>
      <Box sx={{ p: 4, borderRadius: 2, bgcolor: "secondary.main" }}>
        {!service.id ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            Invalid service id
          </Typography>
        ) : (
          <>
            <Typography align="center" variant="h5" style={{ fontWeight: 600 }}>
              {service.title}
            </Typography>
            <Grid container>
              <Grid item xs={6}>
                <Typography align="center" variant="h6">
                  {service.duration}
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography align="center" variant="h6">
                  {service.price}₴
                </Typography>
              </Grid>
            </Grid>
            {orders.length === 0 ? (
              <Typography align="center" variant="h5" sx={{ m: 4 }}>
                No such orders at the moment
              </Typography>
            ) : (
              <>
                <Typography variant="h6" sx={{ m: 2, mt: 6 }}>
                  Orders
                </Typography>
                <TableContainer>
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell>Start time</TableCell>
                        <TableCell>Phone number</TableCell>
                        <TableCell>Email</TableCell>
                        <TableCell>Car make</TableCell>
                        <TableCell>Car model</TableCell>
                        <TableCell>Car year</TableCell>
                        <TableCell>Edit</TableCell>
                        <TableCell>Delete</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {orders.map((o, i) => (
                        <TableRow key={i}>
                          <TableCell>{o.startTime}</TableCell>
                          <TableCell>{o.contact.phoneNumber}</TableCell>
                          <TableCell>{o.contact.email}</TableCell>
                          <TableCell>{o.carMake}</TableCell>
                          <TableCell>{o.carModel}</TableCell>
                          <TableCell>{o.carYear}</TableCell>
                          <TableCell>
                            <Link to={"/admin/orders/edit/" + o.id}>
                              <button>📝</button>
                            </Link>
                          </TableCell>
                          <TableCell>
                            <button
                              onClick={() => {
                                if (
                                  confirm(
                                    "are you sure you want to delete this order?"
                                  )
                                ) {
                                  deleteOrderById(o.id);
                                  window.location.reload();
                                }
                              }}
                            >
                              ❌
                            </button>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </TableContainer>
              </>
            )}
          </>
        )}
      </Box>
    </Box>
  );
};

export default OrdersByService;
