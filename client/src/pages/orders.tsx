import { useState, useEffect } from "react";
import { getAllOrders } from "../api/api";
import { Box, Typography, Grid } from "@mui/material";
import { Order } from "models";

type customGridProps = { title: string; value: string };

const CustomGrid = ({ title, value }: customGridProps) => {
  return (
    <Grid container>
      <Grid item xs={6}>
        <Typography variant="h6">{title}</Typography>
      </Grid>
      <Grid item xs={6}>
        <Typography align="right">{value}</Typography>
      </Grid>
    </Grid>
  );
};

const Orders = () => {
  const [orders, setOrders] = useState<Order[]>([]);

  useEffect(() => {
    getAllOrders().then((data) => {
      setOrders(data);
    });
  }, []);

  return (
    <Grid container>
      {orders.map((o, i) => (
        <Grid key={i} item xs={6}>
          <Box
            key={i}
            sx={{
              m: 4,
              py: 4,
              px: 8,
              borderRadius: 2,
              bgcolor: "secondary.main",
            }}
          >
            <Typography sx={{ mb: 2, fontWeight: 600 }} variant="h5">
              {o.service.title}
            </Typography>
            <CustomGrid title={"Start time:"} value={o.startTime} />
            <CustomGrid title={"Duration:"} value={o.service.duration} />
            <Typography sx={{ mt: 4, mb: 2 }} variant="h5">
              Contacts
            </Typography>
            <CustomGrid
              title={"Phone number: "}
              value={o.contact.phoneNumber}
            />
            {o.contact.email ? (
              <CustomGrid title={"Email: "} value={o.contact.email} />
            ) : null}
            <Typography sx={{ mt: 4, mb: 2 }} variant="h5">
              Car
            </Typography>
            <CustomGrid title={"Make: "} value={o.carMake} />
            <CustomGrid title={"Model: "} value={o.carModel} />
            <CustomGrid title={"Year: "} value={o.carYear.toString()} />
          </Box>
        </Grid>
      ))}
    </Grid>
  );
};

export default Orders;
