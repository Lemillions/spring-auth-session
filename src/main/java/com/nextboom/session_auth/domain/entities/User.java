package com.nextboom.session_auth.domain.entities;

import java.util.UUID;

import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.nextboom.session_auth.domain.value_objects.Email;

import jakarta.persistence.AttributeOverride;
import jakarta.persistence.Column;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "tb_user")
public class User {
  private static final BCryptPasswordEncoder passwordEncoder = new BCryptPasswordEncoder();
  public static final int MAX_SESSIONS = 5;
  @Id
  @GeneratedValue(strategy = GenerationType.UUID)
  @Column(name = "id")
  private UUID id;

  @Column(name = "name")
  private String name;

  @Embedded
  @AttributeOverride(name = "value", column = @Column(name = "email", unique = true))
  private Email email;

  @Column(name = "password")
  private String password;

  public UUID getId() {
    return id;
  }

  public void setId(UUID id) {
    this.id = id;
  }

  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public Email getEmail() {
    return email;
  }

  public void setEmail(Email email) {
    this.email = email;
  }

  @JsonIgnore
  public String getPassword() {
    return password;
  }

  public void setPassword(String password) {
    this.password = password;
  }


  public boolean checkPassword(String password) {
    return User.passwordEncoder.matches(password, this.password);
  }

  public static String encodePassword(String password) {
    return User.passwordEncoder.encode(password);
  }

  public static User create(String name, Email email, String password) {
    User user = new User();
    user.setId(UUID.randomUUID());
    user.setName(name);
    user.setEmail(email);
    user.setPassword(encodePassword(password));
    return user;
  }
}
