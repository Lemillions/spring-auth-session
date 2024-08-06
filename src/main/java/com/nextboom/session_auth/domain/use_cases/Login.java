package com.nextboom.session_auth.domain.use_cases;

import java.util.HashMap;
import java.util.Map;
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

    this.addToSessionToUserSessions(user.getId().toString(), token);

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
    String userSessionListKey = "user_session_list:" + userId;
    Long userSessionListSize = redisTemplate.opsForList().size(userSessionListKey);

    return userSessionListSize >= User.MAX_SESSIONS;
  }

  private void removeOldestSession(String userId) {
    //TODO: ANALISAR POSSIBILIDADE DE SALVAR NO POSTGRES A LISTA DE SESSÕES APAGADAS
    String userSessionListKey = "user_session_list:" + userId;
    String oldestSessionKey = redisTemplate.opsForList().rightPop(userSessionListKey);
    redisTemplate.delete(Session.DATA_PREFIX + oldestSessionKey);
  }

  private String generateToken() {
    String token = UUID.randomUUID().toString();
    return token;
  }

  private void addToSessionToUserSessions(String userId, String token) {
    String userSessionListKey = "user_session_list:" + userId;
    redisTemplate.opsForList().leftPush(userSessionListKey, token);
  }
}