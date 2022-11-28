import { Link } from "react-router-dom";
import { Box, Grid, Typography } from "@mui/material";

const AdminPanel = () => {
  return (
    <>
      <Box
        sx={{
          m: 6,
          px: 4,
          py: 8,
          borderRadius: 4,
          bgcolor: "secondary.main",
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Link to="/admin/sql">
          <Typography align="center" variant="button">
            execute raw sql
          </Typography>
        </Link>
      </Box>
      <Box
        sx={{
          m: 6,
          px: 4,
          py: 8,
          borderRadius: 4,
          bgcolor: "secondary.main",
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Link to="/admin/reports">
          <Typography align="center" variant="button">
            see monthly reports
          </Typography>
        </Link>
      </Box>
      <Box
        sx={{
          m: 6,
          px: 4,
          py: 8,
          borderRadius: 4,
          bgcolor: "secondary.main",
        }}
      >
        <Grid container spacing={4}>
          <Grid
            item
            xs={12}
            sx={{
              display: "flex",
              justifyContent: "space-around",
              alignItems: "center",
            }}
          >
            <Typography align="center" variant="button">
              View data for:{" "}
            </Typography>
          </Grid>
          <Grid
            item
            xs={3}
            sx={{
              display: "flex",
              justifyContent: "space-around",
              alignItems: "center",
            }}
          >
            <Link to="/admin/cars">
              <Typography align="center" variant="button">
                cars
              </Typography>
            </Link>
          </Grid>
          <Grid
            item
            xs={3}
            sx={{
              display: "flex",
              justifyContent: "space-around",
              alignItems: "center",
            }}
          >
            <Link to="/admin/services">
              <Typography align="center" variant="button">
                services
              </Typography>
            </Link>
          </Grid>
          <Grid
            item
            xs={3}
            sx={{
              display: "flex",
              justifyContent: "space-around",
              alignItems: "center",
            }}
          >
            <Link to="/admin/orders">
              <Typography align="center" variant="button">
                orders
              </Typography>
            </Link>
          </Grid>
          <Grid
            item
            xs={3}
            sx={{
              display: "flex",
              justifyContent: "space-around",
              alignItems: "center",
            }}
          >
            <Link to="/admin/contacts">
              <Typography align="center" variant="button">
                contacts
              </Typography>
            </Link>
          </Grid>
        </Grid>
      </Box>
    </>
  );
};

export default AdminPanel;
