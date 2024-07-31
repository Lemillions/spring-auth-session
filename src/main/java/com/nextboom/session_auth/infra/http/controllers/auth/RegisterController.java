package com.nextboom.session_auth.infra.http.controllers.auth;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.nextboom.session_auth.domain.entities.User;
import com.nextboom.session_auth.domain.use_cases.Register;
import com.nextboom.session_auth.infra.http.dtos.RegisterDto;

import jakarta.validation.Valid;

@RestController
@RequestMapping("/auth")
public class RegisterController {
  @Autowired
  private Register register;

  @PostMapping("/register")
  public ResponseEntity<User> handle(@RequestBody @Valid RegisterDto body) {
    User user = register.execute(body);

    return ResponseEntity.status(HttpStatus.CREATED).body(user);
  }
}
