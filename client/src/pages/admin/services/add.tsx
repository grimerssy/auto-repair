import AddServiceComponent from "../../../components/addService";
import { Box } from "@mui/material";

const EditService = () => {
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
        <AddServiceComponent />
      </Box>
    </Box>
  );
};

export default EditService;
