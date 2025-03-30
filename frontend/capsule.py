import pygame

CAPSULE_COLOR = "#647AA3"
PIPE_COLOR = "#FFFFFF"
BG_COLOR = "#24282F"
SPEED = 3

class Capsule:
    def __init__(self, x, y, width, height, trapeze_height):
        self.x = x
        self.y = y
        self.width = width
        self.height = height

        self.trapeze_height = trapeze_height

        self.top_trapeze = [
            (0, self.trapeze_height),
            (self.width, self.trapeze_height),
            (self.width - self.trapeze_height, 0),
            (self.trapeze_height, 0)
        ]
        self.bottom_trapeze = [
            (0, self.height - self.trapeze_height),
            (self.width, self.height - self.trapeze_height),
            (self.width - self.trapeze_height, self.height),
            (self.trapeze_height, self.height)
        ]

        self.y_offset = 0
        self.max_offset = self.height//2 - self.trapeze_height

        self.anim_state = 0 # [0: OPENED, 1: CLOSING, 2:CLOSED, 3:OPENING]

        self.active = False
    
    def draw(self, screen):
        moved_top_trapeze = [(self.x + point[0], self.y + point[1] + self.y_offset) for point in self.top_trapeze]
        moved_bottom_trapeze = [(self.x + point[0], self.y + point[1] - self.y_offset) for point in self.bottom_trapeze]

        pygame.draw.rect(screen, BG_COLOR, (self.x, self.y, self.width, self.trapeze_height + self.y_offset))
        rect = (self.x, self.y + self.height - self.trapeze_height - self.y_offset, self.width, self.trapeze_height + self.y_offset)
        pygame.draw.rect(screen, BG_COLOR, rect)

        size = (15, self.height - 2* self.trapeze_height - 2*self.y_offset)
        pygame.draw.rect(screen, PIPE_COLOR, pygame.Rect((self.x + 15, self.y + self.trapeze_height + self.y_offset), size))
        pygame.draw.rect(screen, PIPE_COLOR, pygame.Rect((self.x + self.width - 30, self.y + self.trapeze_height + self.y_offset), size))

        pygame.draw.polygon(screen, CAPSULE_COLOR, moved_top_trapeze)
        pygame.draw.polygon(screen, CAPSULE_COLOR, moved_bottom_trapeze)
    
    def animate(self):
        if self.anim_state == 0:
            self.y_offset = 0
        elif self.anim_state == 2:
            self.y_offset = self.max_offset
        elif self.anim_state == 1:
            self.y_offset += SPEED
            if self.y_offset >= self.max_offset:
                self.anim_state = 2
        elif self.anim_state == 3:
            self.y_offset -= SPEED
            if self.y_offset <= 0:
                self.anim_state = 0
    
    def set_anim_state(self, state):
        self.anim_state = state