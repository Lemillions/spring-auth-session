package com.nextboom.session_auth.infra.http.dtos;


import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Email;

public record LoginDto(
  @NotBlank(message = "O email não pode estar em branco")
  @Email(message = "Email inválido")
  String email,

  @NotBlank(message = "A senha não pode estar em branco")
  String password
) {
  
}
