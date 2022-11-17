import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getOrderById } from "../../../api/api";
import EditOrderComponent from "../../../components/editOrder";
import { Box, Grid, Typography } from "@mui/material";
import { Order } from "models";

const EditOrder = () => {
  const { id } = useParams();
  const [order, setOrder] = useState<Order>({
    id: "",
    service: {
      id: "",
      title: "",
      price: 0,
      duration: "",
    },
    worker: {
      id: "",
      firstName: "",
      middleName: null,
      lastName: "",
      dateOfBirth: "",
      startTime: "",
      endTime: "",
    },
    car: {
      vin: "",
      contact: {
        id: "",
        phoneNumber: "",
        email: null,
      },
      make: "",
      model: "",
      year: 0,
    },
    startTime: "",
  });

  useEffect(() => {
    getOrderById(id || "").then((data) => setOrder(data));
  }, []);

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
      }}
    >
      <Box
        sx={{
          p: 4,
          my: 4,
          width: 0.6,
          bgcolor: "secondary.main",
          borderRadius: 2,
        }}
      >
        {!order.id ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            Invalid order id
          </Typography>
        ) : (
          <>
            <Typography
              variant="h5"
              align="center"
              sx={{ my: 4, fontWeight: 600 }}
            >
              {order.service.title}
            </Typography>
            <Grid container spacing={6} sx={{ mb: 4 }}>
              <Grid item xs={6}>
                <Typography variant="h6" align="center">
                  duration: {order.service.duration}
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography variant="h6" align="center">
                  price: {order.service.price}₴
                </Typography>
              </Grid>
            </Grid>
            <EditOrderComponent order={order} />
          </>
        )}
      </Box>
    </Box>
  );
};

export default EditOrder;
