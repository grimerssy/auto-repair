import { useState, useEffect } from "react";
import {
  getAllOrders,
  deleteOrderById,
  deleteOrdersByIds,
  getReceiptByIds,
} from "../../../api/api";
import {
  Box,
  Typography,
  Checkbox,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { Link } from "react-router-dom";
import { Order } from "models";

const OrdersByService = () => {
  const [orders, setOrders] = useState<Order[]>([]);
  const [ids, setIds] = useState<string[]>([]);
  const [pdf, setPdf] = useState("");
  const [isGroupDeleteLoading, setIsGroupDeleteLoading] = useState(false);
  const [checked, _] = useState<any>({});

  const handleDelete = (id: string) => {
    if (confirm("are you sure you want to delete this order?")) {
      deleteOrderById(id);
      window.location.reload();
    }
  };

  const handleGroupDelete = () => {
    setIsGroupDeleteLoading(true);
    if (confirm("are you sure you want to delete these orders?")) {
      deleteOrdersByIds(ids);
      setIsGroupDeleteLoading(false);
      window.location.reload();
    }
  };

  useEffect(() => {
    getAllOrders().then((data) => {
      setOrders(data);
    });
  }, []);
  useEffect(() => {
    if (!ids.length) {
      return;
    }
    getReceiptByIds(ids).then((blob) => setPdf(URL.createObjectURL(blob)));
  }, [ids]);

  return (
    <Box sx={{ m: 4, display: "flex", justifyContent: "center" }}>
      <Box sx={{ p: 4, borderRadius: 2, bgcolor: "secondary.main" }}>
        {orders.length === 0 ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            No such orders at the moment
          </Typography>
        ) : (
          <>
            <Box sx={{ display: "flex", justifyContent: "space-between" }}>
              <Typography variant="h6" sx={{ m: 2 }}>
                Orders
              </Typography>
              <LoadingButton
                type="submit"
                variant="text"
                loading={isGroupDeleteLoading}
                disabled={isGroupDeleteLoading || !ids.length}
                onClick={handleGroupDelete}
              >
                Delete selected
              </LoadingButton>
              <a
                href={ids.length ? pdf : "javascript: void(0)"}
                target="_blank"
              >
                <LoadingButton
                  type="submit"
                  variant="text"
                  disabled={isGroupDeleteLoading || !ids.length}
                >
                  Print receipt for selected
                </LoadingButton>
              </a>
            </Box>
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Service</TableCell>
                    <TableCell>Price</TableCell>
                    <TableCell>Duration</TableCell>
                    <TableCell>Start time</TableCell>
                    <TableCell>Phone number</TableCell>
                    <TableCell>Email</TableCell>
                    <TableCell>Car make</TableCell>
                    <TableCell>Car model</TableCell>
                    <TableCell>Car year</TableCell>
                    <TableCell>Worker</TableCell>
                    <TableCell>Edit</TableCell>
                    <TableCell>Delete</TableCell>
                    <TableCell>Select</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {orders.map((o, i) => (
                    <TableRow key={i}>
                      <TableCell>{o.service.title}</TableCell>
                      <TableCell>{o.service.price}</TableCell>
                      <TableCell>{o.service.duration}</TableCell>
                      <TableCell>{o.startTime}</TableCell>
                      <TableCell>{o.car.contact.phoneNumber}</TableCell>
                      <TableCell>{o.car.contact.email}</TableCell>
                      <TableCell>{o.car.make}</TableCell>
                      <TableCell>{o.car.model}</TableCell>
                      <TableCell>{o.car.year}</TableCell>
                      <TableCell>
                        {o.worker.middleName
                          ? o.worker.firstName +
                          " " +
                          o.worker.middleName +
                          " " +
                          o.worker.lastName
                          : o.worker.firstName + " " + o.worker.lastName}
                      </TableCell>
                      <TableCell>
                        <Link to={"/admin/orders/edit/" + o.id}>
                          <button>📝</button>
                        </Link>
                      </TableCell>
                      <TableCell>
                        <button onClick={() => handleDelete(o.id)}>❌</button>
                      </TableCell>
                      <TableCell>
                        <Checkbox
                          disabled={isGroupDeleteLoading}
                          onClick={() => {
                            checked[o.id] = !checked[o.id];
                            setIds(
                              Object.keys(checked).filter((x) => checked[x])
                            );
                          }}
                        />
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </>
        )}
      </Box>
    </Box>
  );
};

export default OrdersByService;
