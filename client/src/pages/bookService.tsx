import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService } from "../api/api";
import BookNoAuth from "../components/bookNoAuth";
import { Box, Grid, Typography } from "@mui/material";
import { Service } from "models";

const BookService = () => {
  const { id } = useParams();
  const [service, setService] = useState<Service>({
    id: "",
    title: "",
    price: 0,
    duration: "",
  });

  useEffect(() => {
    getService(id || "").then((data) => setService(data));
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
          mt: 4,
          width: 0.6,
          bgcolor: "secondary.main",
          borderRadius: 2,
        }}
      >
        {!service.id ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            Invalid service id
          </Typography>
        ) : (
          <>
            <Typography
              variant="h5"
              align="center"
              sx={{ my: 4, fontWeight: 600 }}
            >
              {service.title}
            </Typography>
            <Grid container spacing={6} sx={{ mb: 4 }}>
              <Grid item xs={6}>
                <Typography variant="h6" align="center">
                  duration: {service.duration}
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography variant="h6" align="center">
                  price: {service.price}₴
                </Typography>
              </Grid>
            </Grid>
            <BookNoAuth serviceId={service.id} />
          </>
        )}
      </Box>
    </Box>
  );
};

export default BookService;
