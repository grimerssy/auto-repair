import { Link } from "react-router-dom";
import { useState, useEffect } from "react";
import { getAllServices } from "../../../api/api";
import { Service } from "../../../models";
import { Box, Typography, Grid } from "@mui/material";

const Orders = () => {
  const [services, setServices] = useState<Service[]>([]);

  useEffect(() => {
    getAllServices().then((data) => {
      setServices(data);
    });
  }, []);

  return (
    <Box
      sx={{
        m: 6,
        px: 4,
        py: 8,
        borderRadius: 4,
        bgcolor: "secondary.main",
      }}
    >
      <Grid container spacing={6}>
        {services.map((s, i) => (
          <Grid key={i} item xs={4}>
            <Link to={"/admin/orders/" + s.id}>
              <Typography align="center" variant="h5">
                {s.title}
              </Typography>
            </Link>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
};

export default Orders;
