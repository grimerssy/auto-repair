import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getCarByVin } from "../../../api/api";
import EditCarComponent from "../../../components/editCar";
import { Box, Typography } from "@mui/material";
import { Car } from "models";

const EditCar = () => {
  const { vin } = useParams();

  const [car, setCar] = useState<Car>({
    vin: "",
    make: "",
    model: "",
    year: 0,
    contact: { id: "", phoneNumber: "", email: null },
  });

  useEffect(() => {
    getCarByVin(vin || "").then((data) => setCar(data));
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
        {!car.vin ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            Invalid car vin
          </Typography>
        ) : (
          <EditCarComponent car={car} />
        )}
      </Box>
    </Box>
  );
};

export default EditCar;
