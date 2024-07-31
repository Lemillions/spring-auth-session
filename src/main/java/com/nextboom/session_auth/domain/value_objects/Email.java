package com.nextboom.session_auth.domain.value_objects;

import java.util.regex.Pattern;

import com.fasterxml.jackson.annotation.JsonValue;
import com.nextboom.session_auth.infra.exceptions.BadRequestException;

import jakarta.persistence.Embeddable;

@Embeddable
public class Email {
  private String value;

  public Email() {
  }

  public Email(String value) {
    setValue(value);
  }

  @JsonValue
  public String getValue() {
    return value;
  }

  public void setValue(String value) {
    if (!hasValidFormat(value)) {
      throw new BadRequestException("Email inválido");
    }
    this.value = value;
  }

  private boolean hasValidFormat(String email) {
    return getMailPattern().matcher(email).matches();
  }

  private Pattern getMailPattern() {
    return Pattern.compile(
        "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,6}$");

  }
}
