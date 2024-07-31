package com.nextboom.session_auth.infra.http.dtos;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import jakarta.validation.constraints.Email;

public record RegisterDto(
    @NotBlank(message = "O email não pode estar em branco") 
    @Email(message = "Email inválido") 
    String email, 
    
    @NotBlank(message = "A senha não pode estar em branco") 
    @Size(min = 6, max = 24, message = "A senha deve ter no mínimo 6 caracteres")
    String password, 
    
    @NotBlank(message = "O nome não pode estar em branco") 
    String name) {

}