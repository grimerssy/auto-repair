import { getAllServices, searchService } from "../api/api";
import { useState, useEffect } from "react";
import ServiceComponent from "../components/service";
import { Box, Grid, TextField } from "@mui/material";
import { Service } from "../models";

const Home = () => {
  const [title, setTitle] = useState("");
  const [services, setServices] = useState<Service[]>([]);

  useEffect(() => {
    searchService(title).then((data) => {
      setServices(data);
    });
  }, [title]);

  return (
    <Box
      sx={{
        mx: 6,
        my: 2,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <TextField
        label="Search"
        value={title}
        onChange={(e) => {
          setTitle(e.target.value);
        }}
        margin="normal"
        sx={{
          width: 0.6,
          my: 4,
          backgroundColor: "secondary.main",
        }}
      />
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
