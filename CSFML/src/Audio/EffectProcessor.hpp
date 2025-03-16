#pragma once

typedef void (*sfEffectProcessor)(const float *inputFrames,
                                  unsigned int *inputFrameCount,
                                  float *outputFrames,
                                  unsigned int *outputFrameCount,
                                  unsigned int frameChannelCount,
                                  void *user_data);
