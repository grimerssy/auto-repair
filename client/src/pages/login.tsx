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
      <Box
        sx={{
          p: 4,
          mt: 4,
          width: 0.6,
          bgcolor: "secondary.main",
          borderRadius: 2,
          display: "flex",
          justifyContent: "center",
        }}
      >
        <Typography variant="button" align="center">
          or
          <Link to="/auth/signup">{" sign up "}</Link>
          if you haven't already
        </Typography>
      </Box>
    </Box>
  );
};

export default Login;
