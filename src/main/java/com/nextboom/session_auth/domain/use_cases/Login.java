package com.nextboom.session_auth.domain.use_cases;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.UUID;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

import com.nextboom.session_auth.domain.entities.User;
import com.nextboom.session_auth.domain.value_objects.Email;
import com.nextboom.session_auth.infra.exceptions.BadRequestException;
import com.nextboom.session_auth.infra.http.RequestInfo;
import com.nextboom.session_auth.infra.http.dtos.LoginDto;
import com.nextboom.session_auth.infra.redis.Session;
import com.nextboom.session_auth.infra.repositories.UserRepository;

import java.time.Duration;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.time.format.DateTimeFormatter;

@Service
public class Login {
  @Autowired
  private UserRepository userRepository;

  @Autowired
  private RedisTemplate<String, String> redisTemplate;

  public String execute(LoginDto input, RequestInfo requestInfo) {
    User user = userRepository.findByEmail(new Email(input.email())).orElseThrow(
        () -> new BadRequestException("Email ou senha inválidos"));

    if (!user.checkPassword(input.password())) {
      throw new BadRequestException("Email ou senha inválidos");
    }

    if (hasReachedSessionLimit(user.getId().toString())) {
      removeOldestSession(user.getId().toString());
    }

    String token = generateToken();

    Session session = Session.create(token, user.getId().toString(), requestInfo);

    redisTemplate.opsForValue().set("session:" + token, user.getId().toString(), Duration.ofHours(1));

    Map<String, Object> sessionData = new HashMap<>();
    sessionData.put("userId", session.getUserId());
    sessionData.put("ip", requestInfo.getIp());
    sessionData.put("userAgent", requestInfo.getUserAgent());
    sessionData.put("createdAt", session.getCreatedAt());
    sessionData.put("expiresAt", session.getExpiresAt());
    sessionData.put("lastAccessAt", session.getLastAccessAt());

    redisTemplate.opsForHash().putAll("session_data:" + token, sessionData);

    return token;
  }

  private Boolean hasReachedSessionLimit(String userId) {
    Set<String> keys = redisTemplate.keys("session:*");
    int sessionCount = 0;

    if (keys != null) {
      for (String key : keys) {
        String value = redisTemplate.opsForValue().get(key);
        if (userId.equals(value)) {
          sessionCount++;
        }
      }
    }
    return sessionCount >= User.MAX_SESSIONS;
  }

  private void removeOldestSession(String userId) {
    Set<String> keys = redisTemplate.keys("session:*");
    String oldestSessionKey = null;
    Long oldestSessionCreatedAt = Long.MAX_VALUE;

    if (keys != null) {
      for (String key : keys) {
        String value = redisTemplate.opsForValue().get(key);
        if (userId.equals(value)) {
          String sessionDataKey = "session_data:" + key.substring(8);
          String createdAt = (String) redisTemplate.opsForHash().get(sessionDataKey, "createdAt");

          LocalDateTime createdAtDateTime = LocalDateTime.parse(createdAt, DateTimeFormatter.ISO_DATE_TIME);
          Long createdAtLong = createdAtDateTime.toInstant(ZoneOffset.UTC).toEpochMilli();

          if (createdAtLong < oldestSessionCreatedAt) {
            oldestSessionKey = key;
            oldestSessionCreatedAt = createdAtLong;
          }
        }
      }
    }

    if (oldestSessionKey != null) {
      redisTemplate.delete(oldestSessionKey);
      redisTemplate.delete("session_data:" + oldestSessionKey.substring(8));
    }
  }

  private String generateToken() {
    String token = UUID.randomUUID().toString();
    return token;
  }
}