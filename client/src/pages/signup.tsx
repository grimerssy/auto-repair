import { Box } from "@mui/material";
import SignupForm from "../components/signup";

const Login = () => {
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
        <SignupForm />
      </Box>
    </Box>
  );
};

export default Login;
