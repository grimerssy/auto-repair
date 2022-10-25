import default_image from "../assets/default.jpg";
import { Link } from "react-router-dom";
import { Box, Grid, Typography } from "@mui/material";
import { Service as Model } from "../models.js";

const Service = (service: Model) => {
  return (
    <Box
      sx={{
        border: "0.1rem solid transparent",
        borderRadius: 2,
        backgroundColor: "secondary.main",
        "&:hover": {
          borderColor: "primary.main",
        },
      }}
    >
      <Link to={"/services/" + service.id}>
        <img
          src={default_image}
          style={{
            height: "100px",
            width: "100%",
            borderTopLeftRadius: "7px",
            borderTopRightRadius: "7px",
          }}
        />
        <Box
          sx={{
            px: 2,
            py: 1,
          }}
        >
          <Grid container spacing={2}>
            <Grid item xs={6}>
              <Typography color="#000000">{service.title}</Typography>
            </Grid>
            <Grid item xs={6}>
              <Typography color="#000000" align="right">
                {service.duration}
              </Typography>
              <Typography color="#000000" align="right">
                {service.price}₴
              </Typography>
            </Grid>
          </Grid>
        </Box>
      </Link>
    </Box>
  );
};

export default Service;
