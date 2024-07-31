package com.nextboom.session_auth.domain.use_cases;

import java.util.Optional;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.nextboom.session_auth.domain.entities.User;
import com.nextboom.session_auth.domain.value_objects.Email;
import com.nextboom.session_auth.infra.exceptions.BadRequestException;
import com.nextboom.session_auth.infra.http.dtos.RegisterDto;
import com.nextboom.session_auth.infra.repositories.UserRepository;


@Service
public class Register {
  @Autowired
  private UserRepository userRepository;

  public User execute(RegisterDto input) {
    Optional<User> emailAlreadyUsed = userRepository.findByEmail(new Email(input.email()));

    if (emailAlreadyUsed.isPresent()) {
      throw new BadRequestException("Email já cadastrado");
    }

    User user = User.create(input.name(), new Email(input.email()), input.password());

    userRepository.save(user);

    return user;
  }
}
