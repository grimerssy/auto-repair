import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService } from "../../../api/api";
import EditServiceComponent from "../../../components/editService";
import { Box, Typography } from "@mui/material";
import { Service } from "models";

const EditService = () => {
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
          <EditServiceComponent service={service} />
        )}
      </Box>
    </Box>
  );
};

export default EditService;
