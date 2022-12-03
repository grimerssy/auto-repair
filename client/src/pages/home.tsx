import { searchService, getPdfPriceList } from "../api/api";
import { useState, useEffect } from "react";
import ServiceComponent from "../components/service";
import { Box, Button, Grid, TextField } from "@mui/material";
import { Service } from "../models";

const Home = () => {
  const [title, setTitle] = useState("");
  const [services, setServices] = useState<Service[]>([]);
  const [pdf, setPdf] = useState("");

  useEffect(() => {
    getPdfPriceList().then((blob) => setPdf(URL.createObjectURL(blob)));
  }, []);
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
      <Box
        textAlign="center"
        sx={{ width: 1, display: "flex", justifyContent: "space-around" }}
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
        <a href={pdf} target="_blank">
          <Button type="button" variant="text" sx={{ my: 4 }}>
            get price list
          </Button>
        </a>
      </Box>
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
