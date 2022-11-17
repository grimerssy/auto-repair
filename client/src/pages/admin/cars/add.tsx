import AddCarComponent from "../../../components/addCar";
import { Box } from "@mui/material";

const AddCar = () => {
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
        <AddCarComponent />
      </Box>
    </Box>
  );
};

export default AddCar;
