const app = require("express")();
const jwt = require("jsonwebtoken");

const rsa_public_key =
  "MIIBCgKCAQEA639u2haGdEoEQ5wf7lfTHEvDW2FuLBNmZgailV3N9L2JCI9NKtk1QOlEW2t6jweRfzjNf7Qs9XZkk6v6hveW2AZAYuhbNxQFT1FOk+Ez2RFVLLNZfIc+sXD0VURkORY7m+CFHfT+pf6hlLrvZONEWdJ1ZmxDtMOH6hTESCOooxdJ8m2+WsA5GuzOvaagZD/P4Gf9uoVjk/+G4jsB3YyaGAu+hs/Xx/ti9xPwFtCiUloJlUxhsDz9my67QMmPype4vv1w2Hhaj3UabCQi5qj4JgSctNayRy73Wk0iXtos1s2S38CUsUuSL7oZWDeIi2pZS0NT7e8cZllAHgSuX8MW+wIDAQAB";

/*---      Verify token       ---*/
const checkAuth = (req, res, next) => {
  try {
    const token = req.headers.authorization.split(" ")[1];
    console.log(token);
    req.userData = jwt.verify(token, rsa_public_key);
    next();
  } catch (err) {
    res.status(401).json({
      message: "Authentication failed"
    });
  }
};

const genToken = (id, priority) =>
  jwt.sign(
    {
      id,
      priority
    },
    rsa_public_key,
    {
      expiresIn: "1h"
    }
  );

  app.get('/', checkAuth, (req, res) => {
    const user = req.userData.user;
    res.status(200).json({
      user: user,
      token: genToken(user)
    });
  });
  




app.listen(3000, error => {
  if (error) {
    console.log(error.message);
  }
  console.log(`Server listening on port 3000`);
});
