package com.nextboom.session_auth.infra.http.controllers.auth;

import java.util.Map;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.nextboom.session_auth.infra.http.Authenticated;
import com.nextboom.session_auth.infra.redis.Session;

import jakarta.servlet.http.HttpServletRequest;

@RestController
@RequestMapping("/auth")
public class MeController {

  @RequestMapping("/me")
  @Authenticated
  public ResponseEntity<Map<String, String>> handle(Session session, HttpServletRequest request) {
    return ResponseEntity.status(HttpStatus.OK).body(session.toMap());
  }
}
