import { Link } from "react-router-dom";
import { Box, Typography } from "@mui/material";
import LoginForm from "../components/login";

const Login = () => {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
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
        <LoginForm />
      </Box>
      <Typography
        variant="overline"
        align="center"
        sx={{ bgcolor: "secondary.main", p: 4, borderRadius: 4, m: 4 }}
      >
        or
        <Link to="/auth/signup">
          <Typography variant="button">{" sign up "}</Typography>
        </Link>
        if you haven't already
      </Typography>
    </Box>
  );
};

export default Login;
