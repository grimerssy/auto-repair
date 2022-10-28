import { Link, useLocation } from "react-router-dom";
import logo from "../assets/logo.png";
import { Box, Typography } from "@mui/material";
import jwt from "jwt-decode";

type Claims = {
  sub: string;
  role: string;
};

const Header = () => {
  const location = useLocation();
  const token = localStorage.getItem("accessToken") || "";
  let role = "";
  try {
    let claims: Claims = jwt(token);
    role = claims.role;
  } catch { }

  return (
    <Box
      sx={{
        px: 4,
        height: "5rem",
        bgcolor: "#2a2836",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
      }}
    >
      <Link to="/">
        <img src={logo} alt="logo" style={{ width: "3.5rem" }} />
      </Link>
      {role === "admin" ? (
        <Link to="/admin">
          <Typography
            variant="button"
            sx={{
              fontSize: 16,
              borderRadius: 2,
              color: "secondary.main",
              "&:hover": {
                color: "primary.main",
              },
            }}
          >
            admin panel
          </Typography>
        </Link>
      ) : null}
      {token ? (
        <Link to="/contacts/edit">
          <Typography
            variant="button"
            sx={{
              fontSize: 16,
              borderRadius: 2,
              color: "secondary.main",
              "&:hover": {
                color: "primary.main",
              },
            }}
          >
            edit contact
          </Typography>
        </Link>
      ) : null}
      <Link to={token ? location.pathname : "/auth/login"}>
        <Typography
          variant="button"
          onClick={
            token
              ? () => {
                localStorage.removeItem("accessToken");
                window.location.reload();
              }
              : () => { }
          }
          sx={{
            fontSize: 16,
            borderRadius: 2,
            color: "secondary.main",
            "&:hover": {
              color: "primary.main",
            },
          }}
        >
          {token ? "log out" : "log in"}
        </Typography>
      </Link>
    </Box>
  );
};

export default Header;
