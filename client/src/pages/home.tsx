import { getAllServices } from "../api/api";
import { useState, useEffect } from "react";
import ServiceComponent from "../components/service";
import { Box, Grid } from "@mui/material";
import { Service } from "../models";

const Home = () => {
  const [services, setServices] = useState<Service[]>([]);

  useEffect(() => {
    getAllServices().then((data) => {
      setServices(data);
    });
  }, []);

  return (
    <Box sx={{ m: 6 }}>
      <Grid container spacing={6}>
        {services.map((s, i: number) => (
          <Grid key={i} item xs={4}>
            <Box>
              <ServiceComponent {...s} />
            </Box>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
};

export default Home;
