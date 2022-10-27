import { Link } from "react-router-dom";
import { Box, Grid, Typography } from "@mui/material";

const AdminPanel = () => {
  return (
    <Box
      sx={{
        m: 6,
        px: 4,
        py: 8,
        bgcolor: "secondary.main",
      }}
    >
      <Grid container spacing={4}>
        <Grid item xs={4}>
          <Link to="/admin/services">
            <Typography align="center" variant="h5">
              services
            </Typography>
          </Link>
        </Grid>
        <Grid item xs={4}>
          <Link to="/admin/orders">
            <Typography align="center" variant="h5">
              orders
            </Typography>
          </Link>
        </Grid>
        <Grid item xs={4}>
          <Link to="/admin/contacts">
            <Typography align="center" variant="h5">
              contacts
            </Typography>
          </Link>
        </Grid>
      </Grid>
    </Box>
  );
};

export default AdminPanel;
