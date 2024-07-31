package com.nextboom.session_auth.infra.exceptions;

import java.time.Instant;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.ObjectError;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;

import jakarta.servlet.http.HttpServletRequest;

@ControllerAdvice
public class ExceptionHandlerController {
  @ExceptionHandler(NotFoundException.class)
  public ResponseEntity<StandardError> notFound(NotFoundException e, HttpServletRequest request) {
    String error = "Not found";
    HttpStatus status = HttpStatus.NOT_FOUND;
    StandardError err = new StandardError(Instant.now(), status.value(), error, e.getMessage(),
        request.getRequestURI());
    return ResponseEntity.status(status).body(err);
  }

  @ExceptionHandler(BadRequestException.class)
  public ResponseEntity<StandardError> badRequest(BadRequestException e, HttpServletRequest request) {
    String error = "Bad request";
    HttpStatus status = HttpStatus.BAD_REQUEST;
    StandardError err = new StandardError(Instant.now(), status.value(), error, e.getMessage(),
        request.getRequestURI());
    return ResponseEntity.status(status).body(err);
  }

  @ExceptionHandler(MethodArgumentNotValidException.class)
  public ResponseEntity<StandardError> handle(MethodArgumentNotValidException e, HttpServletRequest request) {
    String error = "Bad request";
    HttpStatus status = HttpStatus.BAD_REQUEST;

    String errorMessage = e.getBindingResult().getAllErrors().stream()
        .findFirst()
        .map(ObjectError::getDefaultMessage)
        .orElse("Validation failed");

    StandardError standardError = new StandardError(Instant.now(), status.value(), error, errorMessage,
        request.getRequestURI());

    return ResponseEntity.badRequest().body(standardError);
  }

  @ExceptionHandler(UnauthorizedException.class)
  public ResponseEntity<StandardError> unauthorized(UnauthorizedException e, HttpServletRequest request) {
    String error = "Unauthorized";
    HttpStatus status = HttpStatus.UNAUTHORIZED;
    StandardError err = new StandardError(Instant.now(), status.value(), error, e.getMessage(),
        request.getRequestURI());
    return ResponseEntity.status(status).body(err);
  }
}
