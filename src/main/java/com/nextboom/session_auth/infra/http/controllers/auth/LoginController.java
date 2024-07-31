package com.nextboom.session_auth.infra.http.controllers.auth;

import java.util.HashMap;
import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.nextboom.session_auth.domain.use_cases.Login;
import com.nextboom.session_auth.infra.http.RequestInfo;
import com.nextboom.session_auth.infra.http.dtos.LoginDto;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;

@RestController
@RequestMapping("/auth")
public class LoginController {
  @Autowired
  private Login login;

  @PostMapping("/login")
  public ResponseEntity<Map<String, String>> handle(@RequestBody @Valid LoginDto body, HttpServletRequest request) {
    Map<String, String> response = new HashMap<String, String>();

    String token = login.execute(body, new RequestInfo(
      request.getRemoteAddr(),
      request.getHeader("User-Agent")
    ));

    response.put("token", token);

    return ResponseEntity.status(HttpStatus.OK).body(response);
  }
}
