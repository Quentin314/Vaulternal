import pygame

class Loading:
    def __init__(self, screen_size):
        self.x = 0
        self.y = 0
        self.width = screen_size[0]
        self.height = screen_size[1]
        self.active = False
        #self.font = pygame.font.Font("scpfont.ttf", 36)
        self.overlay = pygame.Surface((screen_size))  
        self.overlay.fill((0, 0, 0))  
        self.overlay.set_alpha(128)
        self.image = pygame.image.load("loading.png")
        self.image = pygame.transform.scale(self.image, (self.image.get_width() // 2, self.image.get_height() // 2))
        self.image_rect = self.image.get_rect(center=(self.width // 2, self.height // 2))
        self.rotation_angle = 0
    def draw(self, screen, frame):
        if not self.active:
            return
        screen.blit(self.overlay, (0, 0))
        if frame % 2 == 0:
            self.rotation_angle += 4
        rotated_image = pygame.transform.rotate(self.image, self.rotation_angle)
        rotated_image_rect = rotated_image.get_rect(center=self.image_rect.center)
        screen.blit(rotated_image, rotated_image_rect.topleft)
        