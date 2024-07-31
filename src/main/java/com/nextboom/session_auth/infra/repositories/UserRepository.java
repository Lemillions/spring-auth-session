package com.nextboom.session_auth.infra.repositories;

import java.util.Optional;
import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.nextboom.session_auth.domain.entities.User;
import com.nextboom.session_auth.domain.value_objects.Email;

@Repository
public interface UserRepository extends JpaRepository<User, UUID> {
  Optional<User> findByEmail(Email email);
}
